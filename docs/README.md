# docs

ที่คุยกันระหว่าง frontend และ backend

- `api/client/` — สัญญา API ของเว็บลูกค้า (แนะนำ OpenAPI `openapi.yaml`)
- `api/admin/` — สัญญา API ของ dashboard
- `decisions/` — บันทึกการตัดสินใจ ตั้งชื่อ `NNNN-ชื่อเรื่อง.md` (เช่น `0001-auth.md`)

## วิธีทำงาน
1. ใครต้องการ endpoint ใหม่/แก้ → เปิด branch + PR แก้ไฟล์ใน `api/`
2. อีกฝั่ง review และ approve ก่อนเริ่มเขียนโค้ด
3. merge แล้วถือเป็นสัญญา — frontend mock ตามนี้ได้เลย backend implement ตามนี้
