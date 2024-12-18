# PoliTo Rust Backup

Project for the course *System and Device Programming* at Politecnico di Torino.

The program, written in Rust, works in the background to recognize some shapes drawn with the mouse. Those events can activate some specific actions (e.g. a file backup).


## Usage

The shapes must follow the same drawing order as shown in the figure:

![](assets/shapes.png)

(*source: https://depts.washington.edu/acelab/proj/dollar/index.html*)


As of now we are using the following:
- **rectangle**: initialize backup
- **triangle**: confirm backup
- **delete**: cancel backup
