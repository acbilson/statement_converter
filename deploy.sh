#!/bin/sh
sudo chgrp root target/release/statement_parser && \
sudo chown root target/release/statement_parser && \
sudo chmod 755 target/release/statement_parser && \
sudo scp -f target/release/statement_parser /usr/bin
