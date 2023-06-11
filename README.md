# Fortune Slip Generator

## Getting started
The following needs to be installed on your system:
1. InkScape
2. Fonts (the default template uses Dosis, Hina Mincho and Kaushan Script)

### Windows installation
1. Install InkScape
   - Download the latest version of Inkscape: https://inkscape.org/release/
   - Ensure that the Inkscape installation directory is in your PATH

2. Install Fonts
   - Open the fonts from the `fonts` directory
   - Right click on the font file and select "Install for all users"

### Linux installation

We'll be showing the installation instructions for Ubuntu/Debian, but other distros should
be similar.

1. Install InkScape
   - Use your package manager to install InkScape: `sudo apt install inkscape`
2. Install Fonts
   - Open the fonts from the `fonts` directory
   - Double-click on the font file and click on the "Install" button


### How to use it




## Configuration
### CLI options

- `--interactive` - Run in interactive mode (default)
- Output path
- Output format (svg, pdf)
- Settings path

### Changing the template

The program requires an SVG file as a template.

Any SVG file can be used as a template, as long as:
1. `header`, `luck_level` and all categories are present four times.
2. Unique IDs are used for each element.

The templates have been created using Figma, and these can be found in the
`data/figma_templates` directory. Feel free to edit them to your liking, then refer to the
new template in the `settings.yaml`.
