# frontend-client

เว็บดูดวงแม่หมอกระต่าย (ฝั่งลูกค้า) — **pure UI** ไม่ผูก backend ใด ๆ
ข้อมูลไพ่/คำอ่านตอนนี้เป็น JSON ใน `src/lib/` (ยกมาจากเว็บที่ออนไลน์ https://mae-mor-kratai.vercel.app/)
เมื่อ backend พร้อม ให้เปลี่ยนจุดอ่าน JSON เป็นเรียก API ตามสัญญาใน `docs/api/client/`

```sh
npm ci
npm run dev      # http://127.0.0.1:3010/
npm run build    # static export -> out/
```

## โครงสร้าง
- `src/app/` — หน้า `/` (ดวงรายวัน) และ `/reading` (เปิด 3 ใบ, demo)
- `src/lib/` — ข้อมูลไพ่ + helper motion
- `public/` — ภาพไพ่, ฉากแม่หมอ (iframe `scene/`), วิดีโอสับไพ่
- `docs/` — design notes, ขนาดไพ่

ที่มา: แยกจาก `MaeMorKratai/web/tarot` (commit a1d6fef) วันที่ 2026-11-14 โดยตัดสคริปต์ QA/ingest ภาษา Python และ `.env` ออก
