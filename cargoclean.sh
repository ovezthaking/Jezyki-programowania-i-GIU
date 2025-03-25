#!/bin/bash

# Przeszukuje foldery pasujące do wzorca "Lista[i]" lub "Lista [i]"
for dir in Lista\[*\] "Lista [*]"; do
    # Sprawdza czy folder istnieje
    if [ -d "$dir" ]; then
        echo "Przetwarzam folder: $dir"
        # Wchodzi do głównego folderu
        cd "$dir" || continue
        
        # Przeszukuje podfoldery zad[j]
        for subdir in zad*; do
            if [ -d "$subdir" ]; then
                echo "  Przetwarzam podfolder: $subdir"
                cd "$subdir" || continue
                # Wykonuje cargo clean w podfolderze
                if [ -f "Cargo.toml" ]; then
                    echo "    Wykonuję cargo clean w $subdir"
                    cargo clean
                else
                    echo "    Brak pliku Cargo.toml w $subdir - pomijam"
                fi
                # Wraca do folderu nadrzędnego
                cd ..
            fi
        done
        
        # Wraca do pierwotnej lokalizacji
        cd ..
    fi
done

echo "Zakończono przetwarzanie wszystkich folderów"
