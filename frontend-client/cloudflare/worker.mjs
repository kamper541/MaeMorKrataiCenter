// Static Assets does not serve byte ranges itself. Safari needs them for MP4 playback.
export default {
  async fetch(request, env) {
    const response = await env.ASSETS.fetch(request);
    if (response.status !== 200 || !new URL(request.url).pathname.endsWith('.mp4')) return response;

    const headers = new Headers(response.headers);
    headers.set('Accept-Ranges', 'bytes');
    const range = request.headers.get('Range');
    const ifRange = request.headers.get('If-Range');
    const validatorMatches = !ifRange || (
      !ifRange.startsWith('W/') && (
        ifRange === headers.get('ETag') || ifRange === headers.get('Last-Modified')
      )
    );
    const match = range?.match(/^bytes=(\d*)-(\d*)$/);
    if (request.method !== 'GET' || !validatorMatches || !match || (!match[1] && !match[2])) {
      return new Response(response.body, { status: response.status, headers });
    }

    // The scene videos are small, so slicing one asset keeps this handler simple.
    const data = await response.arrayBuffer();
    const size = data.byteLength;
    const start = match[1] ? Number(match[1]) : Math.max(0, size - Number(match[2]));
    const end = match[1] && match[2] ? Math.min(Number(match[2]), size - 1) : size - 1;
    if (start >= size || end < start || (!match[1] && Number(match[2]) === 0)) {
      headers.set('Content-Range', `bytes */${size}`);
      headers.set('Content-Length', '0');
      return new Response(null, { status: 416, headers });
    }

    headers.set('Content-Range', `bytes ${start}-${end}/${size}`);
    headers.set('Content-Length', String(end - start + 1));
    return new Response(data.slice(start, end + 1), { status: 206, headers });
  },
};
