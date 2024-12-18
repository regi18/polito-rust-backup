# PoliTo Rust Backup

Project for the course *System and Device Programming* at Politecnico di Torino.

The program, written in Rust, works in the background to recognize some shapes drawn with the mouse. Those events can activate some specific actions (e.g. a file backup).


## Usage

1. To initialize a backup:
   - Draw a rectangle starting from the top-left corner in a clockwise direction.

2. To confirm and start the backup:
   - Draw a triangle starting from the vertex in a clockwise direction, or
   - Use the confirmation window that appears on the screen.

3. To cancel the backup:
   - Draw an 'X' starting from the top-left corner, or
   - Use the confirmation window that appears on the screen.

4. To specify the source and destination for the backup, as well as to select file types for backup, modify the `config.toml` file.

### Shapes

The shapes must follow the same drawing order as shown in the figure:

![](assets/shapes.png)

(*source: https://depts.washington.edu/acelab/proj/dollar/index.html*)

As of now we are using the following:
- **rectangle**: initialize backup
- **triangle**: confirm backup
- **delete**: cancel backup
