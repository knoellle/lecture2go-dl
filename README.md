# lecture2go-dl

A wrapper around youtube-dl for downloading videos from [lecture2go](http://lecture2go.uni-hamburg.de/).

# Prerequisites

```
rust
youtube-dl
```

# Installation

```
cargo install --path .
```

# Usage

1. Find video series you want to download.
2. Obtain the RSS feed link. The icon should be located at the top of the video series list.
3. Create a file called `feed.url` containing just the link to the RSS feed in the directory where you want the video files to go.
4. Run lecture2go-dl in that directory.

Simply repeat step 4 when new videos have been added. Videos that have been downloaded already are skipped automatically.
