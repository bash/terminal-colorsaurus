#!/usr/bin/env python3

from pprint import pprint
import subprocess as p
from tempfile import NamedTemporaryFile
import os

TERMINALS = {
    "Alacritty": ["alacritty", "--command"],
    "contour": ["contour", "working-directory", os.getcwd(), "--"],
    "foot": ["foot", "--"],
    "st": ["st", "--"],
    # "WezTerm": ["flatpak", "run", "org.wezfurlong.wezterm", "--"],
    "Kitty": ["kitty", "--"],
    "Konsole": ["konsole", "-e"],
    "Gnome Terminal": ["gnome-terminal", "--wait", "--verbose", "--"],
    "Terminology": ["terminology", "--exec"],
    "rxvt-unicode": ["urxvt", "-e"],
    "xterm": ["xterm", "-e"],
}


for terminal, command in TERMINALS.items():
    print(f"Testing {terminal}...")
    with NamedTemporaryFile() as output_file:
        command_with_args = command + [
            # "bash",
            # "-c",
            # f"sleep 3; cargo run --example query -- {output_file.name}&; sleep 10",
            "cargo",
            "run",
            "--example",
            "query",
            "--",
            output_file.name,
        ]
        print(f"\t-> command: {' '.join(command_with_args)}")
        p.call(
            command_with_args,
            stdout=p.DEVNULL,
            stderr=p.DEVNULL,
        )
        print(f"\t-> output: {output_file.readlines()}")
