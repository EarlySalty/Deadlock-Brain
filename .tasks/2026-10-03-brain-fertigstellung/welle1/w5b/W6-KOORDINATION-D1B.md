# Schreibzuständigkeit mit W6

Kopf hat ein separates Paket W6 beauftragt: dl-bot beantwortet nur DMs und direkte Erwähnungen über den bestehenden Brain-Client mit bot.public, keine Serverguide-Funktionen. Ein frischer Worker beginnt dafür zunächst lesend im Bots-Repo und meldet die benötigten Module an D1b. Er schreibt erst nach geklärter Zuständigkeit Source.

Dein Vorrang bleibt der Zugriffsvertrag aus 13:55, danach Verify-/TempVoice-Sicht und Ton. Du besitzt MCP, dessen HTTP-Routing/Autorisierung, öffentliche Faktentypen und dafür nötige zentrale TOML-/Secretanbindung. Melde früh die tatsächlich benötigten gemeinsamen Dateipfade, insbesondere zentrale Konfigurations-, Secret- und Startmodule, damit D1b W6 getrennte Pfade zuweisen oder ihn bis nach deinem Sourceabschluss warten lassen kann. Bestehende DM-/Message-Eventhandler und Brain-Consumerwiring nicht nebenbei ändern.

Bots-main/Deploy werden nacheinander ausgeführt: zuerst dein geprüftes Paket, W6 erst danach auf dem aktuellen Bots-Stand mit eigenem Gate und normalem Deploy. Kein paralleler current-Wechsel oder Rückdeploy. Brain-main/Deploy bleibt bei W1. Du bist nicht allein im Arbeitsraum; fremde Änderungen erhalten, keine Nachrichten an andere Worker oder Kopf, keine weiteren Threads. humanizer/no-em-dashes anwenden.
