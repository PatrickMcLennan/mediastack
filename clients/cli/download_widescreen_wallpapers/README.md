# download_widescreen_wallpapers

Scans `WIDESCREEN_WALLPAPERS_DIR` for existing images, makes HTTP call to `http_get_widescreen_wallpapers`, downloads any new image in dynamo from the source url.

## Developing

### You need

- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)

### Commands

```
$ docker-compose up dev
```

Compile a dev release into a volume shared with the current directory.

```
$ docker-compose up prod
```

Compile a prod release into a volume shared with the current directory.

```
$ docker-compose up run
```

Creates shared volumes between `WIDESCREEN_WALLPAPERS_DIR` and the current directory, runs prod build and outputs to `WIDESCREEN_WALLPAPERS_DIR`

## Notes

- See `.env.example`!  All args are mandatory.