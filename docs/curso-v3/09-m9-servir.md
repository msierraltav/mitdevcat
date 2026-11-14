# M9 — Servir desde la Pi

## Concepto

Tenés archivos estáticos. Servirlos es la parte fácil; la parte difícil es
**convivir con lo que ya está corriendo** en esa máquina.

Estado real de tu Pi:

```
0.0.0.0:80    Pi-hole (lighttpd)   ← ya ocupado
0.0.0.0:443   Pi-hole (lighttpd)   ← ya ocupado
:22           ssh
```

O sea: **no** podés simplemente "levantar tu página en el 80". Hay tres caminos y
cada uno tiene un costo.

## Los tres caminos

**A. Puerto alto, sin tocar nada.** Tu página en `:8080`. Simple, cero conflicto,
no la ve nadie de afuera. Es el camino para probar.

**B. Tailscale Serve.** Publica `:8080` como HTTPS en tu red privada:

```sh
tailscale serve --bg 8080
tailscale serve status
```

Te da `https://yuki-pi.<tu-tailnet>.ts.net` con certificado real, sin abrir puertos
del router. Para que la vea cualquiera: `tailscale funnel 8080` (ojo: eso es
publicar en internet — se decide, no se asume).

**C. Reverse proxy (Caddy) delante de todo.** Caddy toma 80/443 y reparte:
`/` → tu página, `/admin` → Pi-hole. Es lo más "de servidor de verdad", y exige
entender dónde estaba Pi-hole y moverlo de puerto interno. Se hace cuando el
camino B ya no alcance.

**Regla de este curso:** A para probar, B para mostrarla a personas, C cuando de
verdad quieras dominar la máquina.

## Verificar antes de pelear

```sh
ss -ltnp | grep -E ':80 |:443 |:8080 '   # quién escucha qué
curl -I http://127.0.0.1:8080             # responde?
systemctl list-units --type=service | grep -E 'lighttpd|pihole|caddy'
```

Si `tailscale serve` falla por el puerto 443, ese es exactamente el conflicto con
Pi-hole: mirá `ss` y decidí si lo movés o usás otro puerto.

## Servir los archivos

No necesitás escribir un servidor: **no es el ejercicio**. Opciones, de menor a
mayor ceremonia:

```sh
python3 -m http.server -d build 8080        # prueba rápida
caddy file-server --root build --listen :8080   # si tenés Caddy
```

## Que quede prendido: systemd

Un servidor que se muere cuando cerrás la terminal no es un servidor. Unidad de
usuario (no hace falta root):

```ini
# ~/.config/systemd/user/mitdevcat.service
[Unit]
Description=mitdevcat static site
After=network.target

[Service]
Type=simple
WorkingDirectory=/home/mit/code/mitdevcat
ExecStart=/usr/bin/python3 -m http.server -d build 8080
Restart=always

[Install]
WantedBy=default.target
```

```sh
systemctl --user daemon-reload
systemctl --user enable --now mitdevcat
systemctl --user status mitdevcat
journalctl --user -u mitdevcat -n 20     # los logs, siempre
loginctl enable-linger mit        # para que sobreviva al logout
```

## Ejercicio

1. Servir en `:8080` con el método que elijas.
2. Verificarlo con `curl -I` (esperás `200` y `content-type: text/html`).
3. La unidad de systemd, prendida y con `enable` (arranca sola al boot).
4. `tailscale serve --bg 8080` y abrir la URL desde el teléfono.

## Criterio de aceptación

- [ ] `curl -I http://127.0.0.1:8080` devuelve `200`.
- [ ] `systemctl --user is-enabled mitdevcat` dice `enabled`.
- [ ] Reiniciás la Pi y la página sigue arriba sin que hagas nada.
- [ ] La URL de Tailscale carga desde el teléfono.
- [ ] Pi-hole **sigue funcionando** (no rompiste el DNS de la casa: probalo).

Ese último punto no es broma: si te llevás puesto Pi-hole, la casa se queda sin
internet y el que apaga el fuego sos vos.

## Qué aprendiste

- Que desplegar es negociar con lo que ya existe.
- `systemd` de usuario: servicio, restart, logs, linger.
- Publicar (funnel) es una decisión con consecuencias, no un flag.
