# skiron-dust

Downloads [European dust concentration and dust load forecasts from the University of Athens](https://forecast.uoa.gr/en/forecast-maps/dust/europe) website and combines them into GIF files.

```
Usage: skiron-dust.exe [OPTIONS]

Options:
      --dust-load [<OUTPUT FILE>]
          Download Dust Load data into the given file.
      --dust-concentration [<OUTPUT FILE>]
          Download Dust Concentration data into the given file.
  -f, --fps <FPS>
          Frames per second of the resulting GIF. [default: 5]
  -s, --save-intermediate
          Save the intermediate PNG frame files with their original filenames.
  -h, --help
          Print help
  -V, --version
          Print version
```

Example with specified filenames:

`./skiron-dust --dust-load dust.gif --dust-concentration concentration.gif --fps 5`

Example with default values, dust load only:

`./skiron-dust --dust-load`
