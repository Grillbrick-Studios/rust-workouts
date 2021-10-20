# Workout!

This package is designed to be used while working out. It gives a list of
workouts that are saved in the `data` directory. This is located by default
under `$HOME/.config/workouts` but can be overridden with the
`WORKOUT_CONFIG_DIR` environment variable.

If you want to add more workouts you can easily use the `import/example.yml`
file as a model for yours.

The basic idea is starting with a warmup with time provided in the input file
then doing 20/20/20/60 intervals. So it comes out to 60 seconds of work and 60
seconds of rest, but you are doing three different exercises for each set.

To install this package a simple `cargo install workout` will automatically 
install the executable and the config files.

# CAUTION!

This installs files under your config directory `WORKOUT_CONFIG_DIR` or 
`$HOME/.config/workouts` by default. If you later run `cargo uninstall 
workout` these files will be left and must be removed manually.
