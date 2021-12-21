# download_widescreen_wallpapers

Scans local directory for existing images, makes HTTP call to `http_get_widescreen_wallpapers`, downloads any new image in dynamo from the source url.

## Notes

- Requires the entire endpoint from api gateway, global API key & local image dir. See `.env.example`.
