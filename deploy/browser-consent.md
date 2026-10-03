# Lokale Brave-Freigabe

Der Dienst bestätigt ausschließlich den nativen Brave-Dialog für einen eigenen laufenden Import. Die Prozess- und Socketprüfung verlangt eine einzelne Verbindung des Importprozesses zum vorhandenen Brave. Der AT-SPI-Baum muss den Alert „Allow remote debugging?“ außerhalb eines Webdokuments und darunter genau einen nativen Allow-Knopf enthalten. Die Bestätigung erfolgt über dessen native Aktion. Webseiten, Eingabewerte und Bilder werden nicht gelesen.

Brave braucht die normale Startoption `--force-renderer-accessibility=complete`. `brave-browser-accessible.desktop` wird dafür unter `~/.local/share/applications/brave-browser.desktop` installiert. Die vorhandene Sitzung wurde beim ersten Aktivieren mit `--restore-last-session` wiederhergestellt. Ein direkt ausgeführter Browserstart muss dieselbe Accessibility-Option verwenden.

Die normale JSON-Konfiguration enthält den lokalen AT-SPI-Bus der Desktop-Sitzung. Der Dienst muss dieselbe Host-Prozesssicht wie Brave besitzen. Eine systemd-Usernamespace würde die Prüfung der echten Prozessdateideskriptoren verhindern. `NoNewPrivileges=true` bleibt aktiv.
