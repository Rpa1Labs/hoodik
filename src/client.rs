pub const _DEFAULT: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/index.html"));
pub const _CLIENT: [(&str, &[u8]); 21] = [
("favicon.ico", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/favicon.ico"))),
("index.html", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/index.html"))),
("pwa-512x512.png", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/pwa-512x512.png"))),
("pwa-192x192.png", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/pwa-192x192.png"))),
("sw.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/sw.js"))),
("assets/constants-966312bb.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/constants-966312bb.js"))),
("assets/IndexView-ee818317.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/IndexView-ee818317.js"))),
("assets/SectionFullScreen.vue_vue_type_script_setup_true_lang-b345be76.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/SectionFullScreen.vue_vue_type_script_setup_true_lang-b345be76.js"))),
("assets/main.8066ae06.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/main.8066ae06.js"))),
("assets/SetupLockScreenView-a3c381a6.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/SetupLockScreenView-a3c381a6.js"))),
("assets/cryptfns_bg.wasm", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/cryptfns_bg.wasm"))),
("assets/AppCheckbox.vue_vue_type_script_setup_true_lang-16075b63.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/AppCheckbox.vue_vue_type_script_setup_true_lang-16075b63.js"))),
("assets/cryptfns_bg-335ab2b9.wasm", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/cryptfns_bg-335ab2b9.wasm"))),
("assets/DecryptView-836e55a1.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/DecryptView-836e55a1.js"))),
("assets/TwoFactorView-7cf84dbb.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/TwoFactorView-7cf84dbb.js"))),
("assets/PrivateKeyView-7ace6178.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/PrivateKeyView-7ace6178.js"))),
("assets/TwoFactorView-6b50b406.css", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/TwoFactorView-6b50b406.css"))),
("assets/KeyView-085f8d0a.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/KeyView-085f8d0a.js"))),
("assets/LockView-04639ca3.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/LockView-04639ca3.js"))),
("assets/index-04810a1c.css", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/index-04810a1c.css"))),
("assets/IndexView-fe30df18.js", include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/clients/web/dist/assets/IndexView-fe30df18.js"))),
];
