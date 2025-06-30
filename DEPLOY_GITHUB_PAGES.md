# Despliegue en GitHub Pages - MitDevcat

Esta guía te ayudará a desplegar tu aplicación Dioxus en GitHub Pages de manera sencilla y efectiva.

## 🚀 Despliegue Rápido

¿Quieres desplegar ahora mismo? Usa el script automatizado:

1. **Cambia el `base_path`** en `Dioxus.toml` por el nombre de tu repositorio
2. **Ejecuta el script** `deploy.bat` (ya está creado en tu proyecto)
3. **Configura GitHub Pages** en Settings → Pages → Branch: main → Folder: /docs

¡Eso es todo! Tu app estará en `https://tu-usuario.github.io/tu-repositorio/`

## Prerrequisitos

1. **Instalar Dioxus CLI** (si no lo tienes instalado):
   ```bash
   cargo install dioxus-cli
   ```

2. **Tener tu repositorio en GitHub** con el código fuente
3. **Configurar GitHub Pages** en tu repositorio (Settings → Pages → Source: Deploy from a branch → Branch: main → Folder: /docs)

## Configuración del Proyecto

### 1. Configurar Dioxus.toml ✅

Tu archivo `Dioxus.toml` ya está configurado correctamente para GitHub Pages:

```toml
[application]
name = "mitdevcat"
default_platform = "web"

[web.app]
title = "MitDevcat"
base_path = "mitdevcat"

[web.watcher]
watch_path = ["src", "assets"]

[web.resource]
dev = true
style = ["assets/main.css", "assets/styles/header.css", "assets/styles/animations.css"]

[web.proxy]

[bundle]
out_dir = "docs"  # GitHub Pages leerá desde la carpeta docs
```

> **Importante**: Reemplaza `"mitdevcat"` en `base_path` con el nombre exacto de tu repositorio en GitHub.

### 2. Verificar la estructura de assets ✅

Tu estructura de assets está perfecta:

```
assets/
├── favicon.ico
├── github.svg
├── header.svg
├── main.css
├── Mit-logo-dark.png
├── Mit-logo-light.png
├── theme-toggle.js
└── styles/
    ├── animations.css
    ├── header.css
    └── hero.css
```

## Proceso de Despliegue

### Paso 1: Construir la aplicación

Ejecuta el siguiente comando para construir tu aplicación en la carpeta `docs`:

```bash
dx bundle --out-dir docs
```

### Paso 2: Mover archivos estáticos

Mueve el contenido estático de `docs/public` a `docs`:

```bash
# En Windows (CMD)
move docs\public\* docs\

# En Windows (PowerShell)
Move-Item -Path "docs\public\*" -Destination "docs\"

# En Linux/macOS
mv docs/public/* docs/
```

### Paso 3: Configurar routing del lado cliente

Crea una copia del archivo `index.html` como `404.html` para que el routing funcione correctamente:

```bash
# En Windows
copy docs\index.html docs\404.html

# En Linux/macOS
cp docs/index.html docs/404.html
```

### Paso 4: Limpiar directorio público vacío

```bash
# En Windows
rmdir docs\public

# En Linux/macOS
rmdir docs/public
```

### Paso 5: Commit y push a GitHub

```bash
git add .
git commit -m "Deploy to GitHub Pages"
git push origin main
```

## Script de Despliegue Automatizado

Para facilitar el proceso, ya tienes creado un script `deploy.bat` en tu proyecto que automatiza todos los pasos:

### Windows (deploy.bat) ✅ Ya creado

```batch
@echo off
echo Construyendo aplicacion...
dx bundle --out-dir docs

echo Moviendo archivos estaticos...
if exist "docs\public" (
    move docs\public\* docs\
    rmdir docs\public
)

echo Configurando routing...
copy docs\index.html docs\404.html

echo Desplegando a GitHub...
git add .
git commit -m "Deploy to GitHub Pages - %date% %time%"
git push origin main

echo Despliegue completado!
pause
```

**Para usar el script simplemente ejecuta:**
```cmd
deploy.bat
```

### Linux/macOS (deploy.sh)

```bash
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
```

## Configuración en GitHub

1. Ve a tu repositorio en GitHub
2. Navega a **Settings** → **Pages**
3. En **Source**, selecciona **Deploy from a branch**
4. En **Branch**, selecciona **main**
5. En **Folder**, selecciona **/docs**
6. Haz clic en **Save**

## Verificación

Después del despliegue, tu aplicación estará disponible en:
```
https://tu-usuario.github.io/nombre-de-tu-repositorio/
```

## Solución de Problemas

### Error 404 en rutas
- Asegúrate de haber creado el archivo `404.html`
- Verifica que `base_path` en `Dioxus.toml` coincida con el nombre de tu repositorio

### Assets no cargan
- Verifica que los archivos estén en la carpeta `docs` después del build
- Asegúrate de que las rutas en tu código Rust usen las constantes `Asset` correctamente

### Cambios no se reflejan
- Puede tomar unos minutos para que GitHub Pages actualice
- Verifica que el commit se haya subido correctamente
- Revisa la pestaña **Actions** en GitHub para ver el estado del despliegue

## Comandos Útiles

```bash
# Desarrollo local
dx serve

# Build para producción
dx bundle --release --out-dir docs

# Verificar archivos generados
ls docs/  # Linux/macOS
dir docs\ # Windows
```

## Notas Adicionales

- GitHub Pages puede tardar unos minutos en actualizar después del push
- Asegúrate de que tu repositorio sea público para usar GitHub Pages gratuito
- Los cambios en assets requieren un nuevo build completo
- Mantén la carpeta `docs` en tu repositorio para el despliegue automático

¡Listo! Tu aplicación Dioxus ahora estará disponible en GitHub Pages. 🚀
