# ascii-moon-phase-rust
A command line program to display moon phases as ASCII art. (This is a vibe coding experiment in Rust.)

    Usage: ascii-moon-phase [OPTIONS]

    Options:
          --size <SIZE>              Height in rows (width is 2*size) [default: 24]
          --hemisphere <HEMISPHERE>  Orientation (north: waxing on RIGHT; south: mirrored) [default: north] [possible values: north, south]
          --date <DATE>              Date to render (YYYY-MM-DD). Default: today (UTC)
          --phase <PHASE>            Phase to render (overrides --date). 0.0=new, 0.5=full, 1.0=new
          --light-char <LIGHT_CHAR>  Character for illuminated area [default: @]
          --dark-char <DARK_CHAR>    Character for dark area [default: .]
          --empty-char <EMPTY_CHAR>  Character outside the disc [default: " "]
          --show-phase               Print the numeric phase after the art
      -h, --help                     Print help
      -V, --version                  Print version