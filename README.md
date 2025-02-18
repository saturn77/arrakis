# Arrakis

This repository is for a simple tooling application for a serial port monitor, with optional
features such as logging and plotting. This is a work in progress application and is also
serving the purpose of desktop gui development with Egui. 

## User Interface

Reflecting modern design styles, there is an option to choose the theme of the application. 

## Threading 

There is a simple background thread passed into the main application state, which may be used
to trigger reading of data from selected serial port. 

