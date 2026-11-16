import assert from 'node:assert/strict';
import test from 'node:test';
import worker from './worker.mjs';

const bytes = Uint8Array.from({ length: 100 }, (_, index) => index);
const env = {
  ASSETS: {
    async fetch(request) {
      return new Response(request.method === 'HEAD' ? null : bytes, {
        headers: { 'Content-Type': 'video/mp4', 'Content-Length': '100', ETag: '"current"' },
      });
    },
  },
};
const fetchVideo = (headers = {}, method = 'GET') => worker.fetch(
  new Request('https://example.com/scene/draw-shuffle.mp4', { headers, method }), env,
);

test('serves exact bytes for bounded, open-ended, suffix and clamped ranges', async () => {
  for (const [range, start, end] of [
    ['bytes=0-1', 0, 1], ['bytes=25-', 25, 99], ['bytes=-10', 90, 99], ['bytes=90-200', 90, 99],
  ]) {
    const response = await fetchVideo({ Range: range });
    assert.equal(response.status, 206);
    assert.equal(response.headers.get('Content-Range'), `bytes ${start}-${end}/100`);
    assert.equal(response.headers.get('Content-Length'), String(end - start + 1));
    assert.deepEqual(new Uint8Array(await response.arrayBuffer()), bytes.slice(start, end + 1));
  }
});

test('unsatisfiable ranges return an empty 416 with the resource size', async () => {
  for (const range of ['bytes=100-', 'bytes=50-10', 'bytes=-0']) {
    const response = await fetchVideo({ Range: range });
    assert.equal(response.status, 416);
    assert.equal(response.headers.get('Content-Range'), 'bytes */100');
    assert.equal((await response.arrayBuffer()).byteLength, 0);
  }
});

test('whole files and unsupported range formats retain a full 200 response', async () => {
  for (const headers of [{}, { Range: 'bytes=invalid' }, { Range: 'bytes=0-1,5-9' }]) {
    const response = await fetchVideo(headers);
    assert.equal(response.status, 200);
    assert.equal(response.headers.get('Accept-Ranges'), 'bytes');
    assert.deepEqual(new Uint8Array(await response.arrayBuffer()), bytes);
  }
});

test('If-Range applies ranges only to the matching strong validator', async () => {
  for (const [validator, status] of [['"current"', 206], ['"stale"', 200], ['W/"current"', 200]]) {
    const response = await fetchVideo({ Range: 'bytes=0-1', 'If-Range': validator });
    assert.equal(response.status, status);
    assert.equal((await response.arrayBuffer()).byteLength, status === 206 ? 2 : 100);
  }
});

test('HEAD retains the full content length and sends no body', async () => {
  const response = await fetchVideo({ Range: 'bytes=0-1' }, 'HEAD');
  assert.equal(response.status, 200);
  assert.equal(response.headers.get('Content-Length'), '100');
  assert.equal((await response.arrayBuffer()).byteLength, 0);
});

test('asset errors pass through without becoming partial video responses', async () => {
  const response = await worker.fetch(new Request('https://example.com/scene/missing.mp4', {
    headers: { Range: 'bytes=0-1' },
  }), { ASSETS: { fetch: async () => new Response('Not found', { status: 404 }) } });
  assert.equal(response.status, 404);
  assert.equal(await response.text(), 'Not found');
});
