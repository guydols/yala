pub const CSS: &str = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    text-decoration: none;
}

body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', sans-serif;
    background: #111827;
    color: #f3f4f6;
    min-height: 100vh;
    overscroll-behavior: none;
}

.container {
    max-width: 672px;
    margin: 0 auto;
}

h1 {
    font-size: 24px;
    font-weight: 500;
    text-align: center;
    color: #f3f4f6;
}

.toptitle {
    padding-top: 16px;
    padding-bottom: 16px;
}

input[type="text"] {
    width: 100%;
    padding: 16px;
    background: #1f2937;
    border: 1px solid #374151;
    border-radius: 8px;
    color: #f3f4f6;
    font-size: 16px;
    margin-bottom: 16px;
    transition: border-color 0.2s;
}

input[type="text"]:focus {
    outline: none;
    border-color: #6b7280;
}

input::placeholder {
    color: #6b7280;
}

button, .btn {
    width: 100%;
    padding: 16px;
    border: none;
    border-radius: 8px;
    font-size: 16px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
}

button:hover, .btn:hover {
    opacity: 0.9;
}

.btn-primary {
    background: #2563eb;
    color: white;
    text-decoration: none;
}

.btn-primary:hover {
    background: #1d4ed8;
}

.btn-danger {
    background: #dc2626;
    color: white;
}

.btn-danger:hover {
    background: #b91c1c;
}

.list-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px;
    border-radius: 8px;
    background: #1f2937;
    border: 1px solid #374151;
    margin-bottom: 12px;
    cursor: pointer;
    text-decoration: none;
    color: inherit;
    transition: background 0.2s;
}

.list-item:hover {
    background: #293548;
}

.list-name {
    font-size: 18px;
    font-weight: 500;
}

.item-count {
    color: #9ca3af;
    margin-right: 12px;
}

.arrow {
    color: #9ca3af;
    font-size: 20px;
}

.fab {
    position: fixed;
    bottom: 30px;
    right: 30px;
    width: 56px;
    height: 56px;
    background: #2563eb;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 28px;
    color: white;
    box-shadow: 0 4px 12px rgba(37, 99, 235, 0.4);
    text-decoration: none;
    font-weight: 300;
    line-height: 1;
}

.fab:hover {
    background: #1d4ed8;
}

.header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;
    padding: 16px 24px;
    background: #1f2937;
    border-bottom: 1px solid #374151;
    position: sticky;
    top: 0;
    z-index: 10;
}

.back-btn, .menu-btn {
    width: 40px;
    height: 40px;
    background: transparent;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    border: none;
    color: #f3f4f6;
    text-decoration: none;
    font-size: 20px;
    transition: background 0.2s;
}

.back-btn:hover, .menu-btn:hover {
    background: #374151;
}

.item {
    display: flex;
    align-items: center;
    padding: 16px 24px;
    border-bottom: 1px solid #1f2937;
    position: relative;
    cursor: grab;
    user-select: none;
    -webkit-user-select: none;
    touch-action: pan-y;
    transition: background 0.2s;
}

.item:hover {
    background: #1a1f2e;
}

.item:active {
    cursor: grabbing;
}

.checkbox {
    width: 24px;
    height: 24px;
    border: 2px solid #4b5563;
    border-radius: 50%;
    margin-right: 12px;
    cursor: pointer;
    flex-shrink: 0;
    transition: all 0.2s;
}

.checkbox.checked {
    background: #2563eb;
    border-color: #2563eb;
    position: relative;
}

.checkbox.checked::after {
    content: '✓';
    position: absolute;
    color: white;
    font-size: 14px;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
}

.item.completed .item-text {
    text-decoration: line-through;
    color: #6b7280;
}

.item-text {
    flex: 1;
    cursor: text;
    padding: 4px;
    color: #f3f4f6;
}

.add-item {
    display: flex;
    align-items: center;
    padding: 16px 24px;
    margin-top: 8px;
}

.add-item form {
    width: 100%;
    display: flex;
    align-items: center;
}

.add-item .checkbox {
    border: 2px solid #4b5563;
    background: transparent;
}

.add-item input {
    margin: 0;
    background: transparent;
    border: none;
    border-bottom: 1px solid #374151;
    padding: 8px 12px;
    color: #f3f4f6;
    flex: 1;
    font-size: 16px;
}

.add-item input:focus {
    outline: none;
    border-bottom-color: #6b7280;
}

.menu {
    position: absolute;
    right: 20px;
    top: 70px;
    background: #1f2937;
    border: 1px solid #374151;
    border-radius: 12px;
    padding: 8px;
    min-width: 224px;
    box-shadow: 0 10px 25px rgba(0,0,0,0.5);
    z-index: 100;
}

.menu-item {
    padding: 12px 16px;
    cursor: pointer;
    border-radius: 8px;
    display: flex;
    align-items: center;
    gap: 12px;
    transition: background 0.2s;
    font-size: 15px;
}

.menu-item:hover {
    background: #374151;
}

.menu-item.danger {
    color: #ef4444;
}

.menu-item svg {
    width: 20px;
    height: 20px;
    flex-shrink: 0;
}

.modal {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.75);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 200;
    padding: 24px;
}

.modal-content {
    background: #1f2937;
    border-radius: 16px;
    padding: 32px;
    max-width: 384px;
    width: 100%;
    border: 1px solid #374151;
}

.modal-title {
    text-align: center;
    margin-bottom: 24px;
    font-size: 20px;
    font-weight: 500;
}

.empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 60vh;
    text-align: center;
}

.empty-icon {
    width: 160px;
    height: 160px;
    background: #1f2937;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 32px;
}

.empty-icon svg {
    width: 80px;
    height: 80px;
    color: #4b5563;
}

.empty-text {
    color: #6b7280;
    font-size: 18px;
    margin-bottom: 32px;
}

.empty-title {
    font-size: 24px;
    font-weight: 500;
    margin-bottom: 24px;
}

::-webkit-scrollbar {
    width: 8px;
}

::-webkit-scrollbar-track {
    background: #1f2937;
}

::-webkit-scrollbar-thumb {
    background: #4b5563;
    border-radius: 4px;
}
"#;
