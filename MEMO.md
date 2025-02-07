コンテナで開発するときは GUI がないので `desktop-lite` を入れる。これがないと `Failed to initialize GTK` というエラーでビルドが通らない。

Docker 上で vite のホットリロードが効かないため、`vite.config.ts` にオプションを追加する。