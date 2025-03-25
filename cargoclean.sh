#!/bin/bash

# Początkowa lokalizacja
START_DIR=$(pwd)

echo "Rozpoczynam przetwarzanie z katalogu: $START_DIR"

# Szukamy folderów Lista* (bez ograniczania do konkretnego wzorca w nawiasach)
for dir in $(find . -maxdepth 1 -type d -name "Lista*"); do
    # Pomijamy bieżący katalog (.)
    if [ "$dir" = "." ]; then
        continue
    fi
    
    echo "Znaleziono folder: $dir"
    cd "$dir" || {
        echo "Nie udało się wejść do $dir"
        continue
    }
    
    # Szukamy podfolderów zaczynających się na zad
    for subdir in $(find . -maxdepth 1 -type d -name "zad*"); do
        if [ "$subdir" = "." ]; then
            continue
        fi
        
        echo "  Przetwarzam podfolder: $subdir"
        cd "$subdir" || {
            echo "    Nie udało się wejść do $subdir"
            continue
        }
        
        # Sprawdzamy czy to projekt Rust
        if [ -f "Cargo.toml" ]; then
            echo "    Wykonuję cargo clean w $(pwd)"
            cargo clean
        else
            echo "    Brak Cargo.toml w $(pwd) - pomijam"
        fi
        
        # Wracamy do folderu nadrzędnego
        cd "$START_DIR/$dir" || {
            echo "    Nie udało się wrócić do $dir"
            continue
        }
    done
    
    # Wracamy do katalogu startowego
    cd "$START_DIR" || {
        echo "Nie udało się wrócić do $START_DIR"
        exit 1
    }
done

echo "Zakończono przetwarzanie"
