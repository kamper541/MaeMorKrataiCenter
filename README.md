# MaeMorKratai Center

Monorepo ของแม่หมอกระต่าย แบ่งเป็น 5 โฟลเดอร์

| โฟลเดอร์ | เจ้าของ | หน้าที่ |
|---|---|---|
| `frontend-client/` | Frontend | เว็บดูดวงแม่หมอ (ลูกค้า) — pure UI |
| `frontend-admin/` | Frontend | Dashboard: ภาพรวมเว็บ, ดูดไอเดียคลิปคู่แข่ง, สั่งงาน content automation |
| `docs/` | ทั้งคู่ | สัญญา API (OpenAPI) + บันทึกการตัดสินใจ — คุยกันที่นี่ |
| `backend-client/` | Backend | API สำหรับเว็บลูกค้า |
| `backend-admin/` | Backend | API สำหรับ dashboard |

## กติกา
- frontend ↔ backend คุยกันผ่าน API ที่เขียนไว้ใน `docs/api/` เท่านั้น ห้าม import โค้ดข้ามโฟลเดอร์
- แก้งานใน branch ของตัวเอง แล้วเปิด PR เข้า `main`
- เปลี่ยนสัญญา API → แก้ `docs/api/` ใน PR แยก ให้อีกฝั่ง approve ก่อน
