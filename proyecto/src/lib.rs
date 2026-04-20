use anchor_lang::prelude::*;

declare_id!("DVPmk6rTxNt1YQnGrbNG6BQBpktUfbmQsQoPtBss6ZW2");

#[program]
mod modulo {
    use super::*;

    pub fn crear_database(ctx: Context<CrearVideojuegoDB>, nombre_db: String) -> Result<()> {
        let db = &mut ctx.accounts.videojuego_db;

        // Validación simple para longitud máxima según #[max_len(30)]
        require!(nombre_db.len() <= 30, ErrorCode::NombreMuyLargo);

        db.nombre_db = nombre_db;
        db.juegos = Vec::new(); // Inicializamos vector vacío de juegos

        Ok(())
    }

    pub fn agregar_videojuego(
        ctx: Context<CrearJuego>,
        juego_nombre: String,
        genero: String,
        estudio: String,
        dificultad: u8,
        calificacion: u8,
    ) -> Result<()> {
        let videojuego_db = &mut ctx.accounts.videojuego_db;
        let juego = &mut ctx.accounts.juego;

        // Validaciones simples de longitud para strings
        require!(juego_nombre.len() <= 30, ErrorCode::NombreMuyLargo);
        require!(genero.len() <= 30, ErrorCode::NombreMuyLargo);
        require!(estudio.len() <= 30, ErrorCode::NombreMuyLargo);

        // Inicializar datos del juego
        juego.nombre = juego_nombre.clone();
        juego.genero = genero;
        juego.estudio = estudio;
        juego.dificultad = dificultad;
        juego.calificacion = calificacion;

        // Agregar la pubkey del juego a la base de datos
        videojuego_db.juegos.push(juego.key());

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("El nombre de la base de datos es demasiado largo.")]
    NombreMuyLargo,
}

#[account]
#[derive(InitSpace)]
pub struct VideojuegoDB {
    #[max_len(30)]
    pub nombre_db: String, // Nombre de la base de datos

    #[max_len(10)]
    pub juegos: Vec<Pubkey>, // Vector con las direcciones de las cuentas de juegos
}

#[account]
#[derive(InitSpace)]
pub struct Juego {
    #[max_len(30)]
    pub nombre: String, // Nombre del videojuego
    #[max_len(30)]
    pub genero: String, // Género del videojuego
    #[max_len(30)]
    pub estudio: String, // Estudio que lo creó

    pub dificultad: u8,   // Dificultad
    pub calificacion: u8, // Calificación
}

#[derive(Accounts)]
pub struct CrearVideojuegoDB<'info> {
    #[account(init, 
    payer = usuario, 
    space = 8 + VideojuegoDB::INIT_SPACE,
    seeds = [b"database", usuario.key().as_ref()],
    bump
    )] // 8 bytes para discriminador + nombre + vector de 10 juegos aprox.
    pub videojuego_db: Account<'info, VideojuegoDB>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(juego_nombre:String)]
pub struct CrearJuego<'info> {
    #[account(init, 
    payer = usuario, 
    space = 8 + Juego::INIT_SPACE,
    seeds = [b"videojuegos", usuario.key().as_ref(), juego_nombre.as_bytes()],
    bump
    )] // 8 disco + 3 strings de max 50 chars + 2 u8
    pub videojuego_db: Account<'info, VideojuegoDB>,

    pub juego: Account<'info, Juego>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}
