# peer-group-grading

Anforderungen an die Bewertungsplattform

Adminbereich

Verwaltung der Lehrkräfte

Anlegen neuer Lehrkräfte
Löschen von Lehrkräften




Lehrendenbereich

Login (Username, PW)
LDAP-Anbindung wäre natürlich wünschenswert
Anlegen von Klassenbereichen (IT3o, IT3e etc.)

Darin können dann Lernfelder angelegt werden


In den LF können die globalen Einzelbefragung eingerichtet werden

Beim Anlegen wird das Standarttemplate genutzt (siehe Anagen > Evaluationsbogen.docx)
Das Standarttemplate kann in seinen Fragen (Teil 1) abgeändert werden


Die Gewichtung (Faktor) der Gruppennote kann global festgelegt werden (Default = 0,5)
Schüler*innengruppen können händisch angelegt werden
Der Gruppen-Moodle-Export (siehe Moodle_Exports) kann eingelesen werden und die Gruppen werden automatisch angelegt anhand der angegebenen Gruppierung

Im Nachhinein können Schüler*innen sowie Gruppen noch hinzugefügt oder entfernt werden


Für jede Person in den Gruppen wird ein eigener Link zum privaten Feedbackformular generiert (möglicherweise mit PW?)

Die Lehrkraft sieht die URLs der Links und kann Sie entsprechend kopieren
Der Link kann auch direkt an die E-Mail aus dem Moodle_Exports versendet werden (Der Versand wird von der Lehrkraft ausgelöst)
Die Lehrkraft ist in der Lage ein ausgefülltes Formular zurückzusetzen (bspw. bei fehlerhaften Eingaben)


Einer Gruppe kann eine Gruppennote zugewiesen werden
Sobald die Gruppennote und alle Peerfeedbackbögen ausgefüllt wurden, wird die Einzelnote bei den Namen der Schüler*innen angezeigt
Die Lehrkraft hat danach ebenfalls vollen Einblick in das schriftliche Feedback und die Kommentare.

Die Gruppennote kann auch ohne vorhandene Peerfeedbackbögen eingetragen werden. Ebenso können auch Peerfeedbackbögen ausgefüllt werden, auch wenn noch keine Gruppennote feststeht. Eine Berechnung der Einzelnote erfolgt erst wenn alle Werte vorliegen.

Schüler*innenbereich

Über den erhaltenen Link können sie auf den Peerfeedbackbogen zugreifen
Das Ausfüllen des Formulars ist nur einmal möglich

Außer die Lehrkraft setzt das Formular zurück


Nach dem Absenden des Formulars werden die Schüler*innen auf eine Warteseite weitergeleitet

„Vielen Dank für dein Feedback! Sobald alle Bögen ausgefüllt wurden und die Gruppennote feststehst erfährst du hier deine Note.“


Sobald die Note feststeht, wird die diese auf der Warteseite angezeigt.
Außerdem sehen die Schüler*innen das Feedback, das sie von den anderen Gruppenmitgliedern erhalten haben (anonymisiert).