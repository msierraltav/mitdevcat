#!/bin/bash

echo "Construyendo aplicación..."
dx bundle --out-dir docs

echo "Moviendo archivos estáticos..."
if [ -d "docs/public" ]; then
    mv docs/public/* docs/
    rmdir docs/public
fi

echo "Configurando routing..."
cp docs/index.html docs/404.html

echo "Desplegando a GitHub..."
git add .
git commit -m "Deploy to GitHub Pages - $(date)"
git push origin main

echo "¡Despliegue completado!"
