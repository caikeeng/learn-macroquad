use macroquad::prelude::*;

#[macroquad::main("Meu Primeiro Jogo")]

async fn main() {
    let mut x = 100.0;
    let mut y = 100.0;
    let speed = 200.0; // pixels por segundo

    loop {
        // 1. Coleta de entrada (input) e atualização da lógica
        let dt = get_frame_time();

        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            x += speed * dt;
        }

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            x -= speed * dt;
        }

        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            y += speed * dt;
        }

        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            y -= speed * dt;
        }

        // 2. Limpeza de tela (corde fundo)
        clear_background(DARKGRAY);

        // 3. Desenho dos elementos na tela
        // Desenhando um retângulo controlado pelo teclado
        draw_rectangle(x, y, 50.0, 50.0, GREEN);

        // Desenhando um texto informativo
        draw_text(
            "Use W, A, S, D ou as Setas para mover o quadrado",
            20.0,
            30.0,
            20.0,
            WHITE,
        );

        // 4. Pausa para o próximo frame
        next_frame().await;
    }
}
