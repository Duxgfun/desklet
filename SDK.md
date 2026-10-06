# Desklet Micro-Widget SDK

Chào mừng bạn đến với **Desklet SDK**! Để đảm bảo triết lý siêu nhẹ và an toàn, Desklet không sử dụng các framework nặng nề cho Widget bên thứ ba, mà sử dụng cấu trúc `.mwidget` (Micro-Widget).

Một gói `.mwidget` thực chất là một thư mục chứa HTML/JS/CSS cơ bản, được Sandbox và render bởi hệ thống Tauri.

## 1. Cấu trúc một Widget

Mỗi Widget bạn đóng góp sẽ nằm trong một thư mục riêng biệt.

```plaintext
my-custom-widget/
├── manifest.json   # Khai báo thông tin widget
├── index.html      # Giao diện hiển thị (UI)
└── script.js       # Logic xử lý (Không bắt buộc)
```

## 2. File `manifest.json`

Đây là file quan trọng nhất để Desklet nhận diện Widget của bạn.

```json
{
  "name": "My Custom Widget",
  "version": "1.0.0",
  "author": "Duxg",
  "description": "Hiển thị câu quote tạo động lực mỗi ngày.",
  "permissions": [
    "network"
  ],
  "default_size": {
    "width": 250,
    "height": 100
  }
}
```
*Lưu ý: Để đảm bảo an toàn, nếu Widget không gọi API ngoài, hãy xóa mảng `permissions`.*

## 3. Lập trình Giao diện (`index.html`)

Bạn có thể viết HTML và CSS nội trú trực tiếp. Hệ thống Desklet sẽ tự động áp dụng các biến CSS (CSS Variables) từ **Styling Engine** vào body của Widget để nó đồng bộ giao diện với toàn bộ hệ thống (Màu nền, màu chữ, Glassmorphism).

```html
<!DOCTYPE html>
<html>
<head>
  <style>
    /* Bạn có thể dùng các biến hệ thống để đồng bộ style */
    body {
      color: var(--desklet-text-color);
      font-family: var(--desklet-font-family);
      margin: 0;
      padding: 10px;
    }
    .quote-text {
      font-size: 14px;
      font-weight: 500;
    }
  </style>
</head>
<body>
  <div class="quote-text" id="quote">Đang tải...</div>
  <script src="./script.js"></script>
</body>
</html>
```

## 4. Bảo mật & Giới hạn (Sandbox)

Desklet chạy tất cả các widget của cộng đồng trong một môi trường **Webview Sandbox** nghiêm ngặt.
- **CẤM** sử dụng hàm `eval()` hoặc `new Function()`.
- **CẤM** truy cập File System của máy tính (Local Disk). 
- Các lệnh như `window.top`, `window.parent` đã bị chặn để tránh can thiệp vào Core Engine.
- Lệnh gọi mạng (Fetch API) bị chặn trừ khi bạn khai báo `"network"` trong `manifest.json`.

## 5. Đưa Widget lên Store (Automated CI/CD)

Desklet sử dụng **GitHub Pages Store** hoàn toàn miễn phí.
1. Fork repository của Desklet.
2. Thêm thư mục Widget của bạn vào `community-widgets/`.
3. Tạo **Pull Request (PR)**.
4. Hệ thống GitHub Actions của chúng tôi sẽ tự động quét mã độc và duyệt PR của bạn nếu an toàn. Sau đó Widget sẽ lập tức có mặt trên kho ứng dụng của hàng ngàn người dùng!

---
*Happy Coding! Cảm ơn bạn đã đóng góp cho cộng đồng mã nguồn mở Desklet.*
