# BMD Speed Editor Zeevonk Controller

This is a very simple controller (with client id `zv-ctrl-bmdse`) client for [Zeevonk](https://github.com/BaukeWestendorp/zeevonk) that converts button and wheel events from a BMD Davinci Resolve Speed Editor using my [bmdse](https://crates.io/crates/bmdse) crate.

To install it, clone the repo and use

```sh
cargo install --path .
```

in the root of the directory. This should install a binary called `zv-ctrl-bmdse` to your path.

## Trigger Mapping

| **Wheel Function**    | **Trigger Id**             |
|-----------------------|----------------------------|
| Wheel Velocity        | `wheel-velocity`           |

| **Button**            | **Trigger Id**             |
|-----------------------|----------------------------|
| Smart Insert          | `button-smart-insert`      |
| Append                | `button-append`            |
| Ripple Overwrite      | `button-ripple-overwrite`  |
| Close Up              | `button-close-up`          |
| Place On Top          | `button-place-on-top`      |
| Source Overwrite      | `button-source-overwrite`  |
| In                    | `button-in`                |
| Out                   | `button-out`               |
| Trim In               | `button-trim-in`           |
| Trim Out              | `button-trim-out`          |
| Roll                  | `button-roll`              |
| Slip Source           | `button-slip-source`       |
| Slip Destination      | `button-slip-destination`  |
| Transition Duration   | `button-transition-duration`|
| Cut                   | `button-cut`               |
| Dissolve              | `button-dissolve`          |
| Smooth Cut            | `button-smooth-cut`        |
| Escape                | `button-escape`            |
| Sync Bin              | `button-sync-bin`          |
| Audio Level           | `button-audio-level`       |
| Full View             | `button-full-view`         |
| Transition            | `button-transition`        |
| Split                 | `button-split`             |
| Snap                  | `button-snap`              |
| Ripple Delete         | `button-ripple-delete`     |
| Cam 1                 | `button-cam-1`             |
| Cam 2                 | `button-cam-2`             |
| Cam 3                 | `button-cam-3`             |
| Cam 4                 | `button-cam-4`             |
| Cam 5                 | `button-cam-5`             |
| Cam 6                 | `button-cam-6`             |
| Cam 7                 | `button-cam-7`             |
| Cam 8                 | `button-cam-8`             |
| Cam 9                 | `button-cam-9`             |
| Live Overwrite        | `button-live-overwrite`    |
| Video Only            | `button-video-only`        |
| Audio Only            | `button-audio-only`        |
| Stop Play             | `button-stop-play`         |
| Source                | `button-source`            |
| Timeline              | `button-timeline`          |
| Shuttle               | `button-shuttle`           |
| Jog                   | `button-jog`               |
| Scroll                | `button-scroll`            |
