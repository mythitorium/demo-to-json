Demo To Json
====
A machine that turns tf2 demo files into json files. Windows only. 

Powered by demo.tf's [parser](https://github.com/demostf/parser) with a bit of code yoinked from their [inspector](https://github.com/demostf/inspector) website.

Usage
-----
1. Download the latest release from the releases tab
2. Run it—
  - With a filepath
  ```
  > demo-to-json "C:\path\to\your\demos\2025-05-12_10-45-19.dem"
  ```

  - With a folder, every demo file within the folder will be processed
  ```
  > demo-to-json "C:\path\to\your\demos"
  ```

  - Or with nothing, you will be prompted to select a file via file dialog
  ```
  > demo-to-json
  ```
  
3. Demo(s) will be parsed and written to json file(s).
4. Json(s) will be saved by the same name (but with a .json extension) and in the same location as the inputted demo(s)

Building
-----
If you truly want to you'll know how.
