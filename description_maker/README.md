# README

The `description_maker` generates a markdown file that has copyable content to help with episode publication. You can find an example of this [here](https://github.com/audrow-nash-podcast/episodes/blob/main/episodes/1-electric-sheep/content.md).

## Installing

It is easiest to use this script by installing it to you system path. You can do this from the root directory of this repository with the following command:

```
cargo install --path description_maker/
```

Once you have this, you should be able to run this program from you command line by typing `description_maker`.

## Usage

You can run the `description_maker` with `-h` or `--help` to see the list of available commands:

```
Makes descriptions for podcast episodes

Usage: description_maker <COMMAND>

Commands:
  init           Makes starter files
  new_episode    Makes a starter episode file
  make_markdown  Generates a markdown file from description info
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

The typical workflow is to

1. Use `init` to create an `episode.yaml` and `podcast.yaml`.

   The `podcast.yaml` is where information regarding your podcast should live. You'll reference this for each episode.

   The `episode.yaml` is where episode specific information lives, such as URLs just for the episode, guest information, and things like the episode number and title.

2. Run `make_markdown` and point it to your `podcast.yaml`, `episode.yaml`, and a text file that has outline information, like [here](https://github.com/audrow-nash-podcast/episodes/blob/main/episodes/1-electric-sheep/outline.txt).

3. Open the generated markdown file and copy paste things into where you'd like them to go (YouTube, Spotify, etc.)

   Note that the Spotify content looks strange, but this is what I had to do to get it to render correctly on different podcasting platforms, like Apple, Google, etc..

If you want to create your own templates or adjust existing ones, you can find the templates in the [templates directory](./templates/).