#!/usr/bin/env bash

echo "Installing..."

sudo cp commander-musl /usr/bin/commander || exit 1

cp ./commander.service ~/.config/systemd/user/commander.service || exit 1

systemctl --user daemon-reload || exit 1

systemctl --user enable commander --now || exit 1

echo "Installation finished."
