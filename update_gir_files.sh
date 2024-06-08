flatpak run --command=cp --filesystem=home org.gnome.Sdk//master /usr/share/gir-1.0/ ./ -r 
rm -rf gir-files
mv gir-1.0 gir-files
mv gir-files/gir-1.2.rnc ./