use anchor_lang::prelude::*;

declare_id!("yJmLTFj6am7eJBxfvfHDSMc1oLDhnk3TJPmSp8iKkF9");

#[program]
pub mod veterinaria_db {
    use super::*;

    pub fn crear_veterinaria(ctx: Context<CrearVeterinaria>, nombre: String) -> Result<()> {
        let veterinaria = &mut ctx.accounts.veterinaria;
        veterinaria.nombre = nombre;
        veterinaria.owner = *ctx.accounts.owner.key;
        veterinaria.mascotas = Vec::new(); // Inicialmente sin mascotas
        Ok(())
    }

    pub fn agregar_mascota(
        ctx: Context<AgregarMascota>,
        especie: String,
        nombre: String,
        owner_mascota: String,
        edad: u8,
    ) -> Result<()> {
        let veterinaria = &mut ctx.accounts.veterinaria;

        // Podemos validar el largo de strings aquí también con require si quieres

        let nueva_mascota = Mascota {
            especie,
            nombre,
            owner: owner_mascota,
            edad,
            vivo: true,
        };

        veterinaria.mascotas.push(nueva_mascota);

        Ok(())
    }

    pub fn eliminar_registro_mascota(
        ctx: Context<EliminarRegistroMascota>,
        nombre_mascota: String,
    ) -> Result<()> {
        let veterinaria = &mut ctx.accounts.veterinaria;

        // Buscar índice de la mascota por nombre
        if let Some(pos) = veterinaria
            .mascotas
            .iter()
            .position(|m| m.nombre == nombre_mascota)
        {
            // Elimina la mascota sin preservar el orden
            veterinaria.mascotas.swap_remove(pos);
            Ok(())
        } else {
            Err(ErrorCode::MascotaNoEncontrada.into())
        }
    }

    pub fn ver_mascotas(ctx: Context<VerMascotas>) -> Result<()> {
        let veterinaria = &ctx.accounts.veterinaria;

        if veterinaria.mascotas.is_empty() {
            msg!("No hay mascotas registradas en la veterinaria.");
            return Ok(());
        }

        msg!(
            "Listado de mascotas en la veterinaria '{}':",
            veterinaria.nombre
        );

        for (i, mascota) in veterinaria.mascotas.iter().enumerate() {
            msg!(
                "Mascota #{}: Nombre: {}, Especie: {}, Dueño: {}, Edad: {}, Vivo: {}",
                i + 1,
                mascota.nombre,
                mascota.especie,
                mascota.owner,
                mascota.edad,
                mascota.vivo
            );
        }

        Ok(())
    }

    pub fn cambiar_estado(ctx: Context<CambiarEstado>, nombre_mascota: String) -> Result<()> {
        let veterinaria = &mut ctx.accounts.veterinaria;

        // Buscar la mascota por nombre
        if let Some(mascota) = veterinaria
            .mascotas
            .iter_mut()
            .find(|m| m.nombre == nombre_mascota)
        {
            mascota.vivo = false;
            msg!(
                "Estado cambiado: la mascota '{}' ahora está marcada como no viva.",
                nombre_mascota
            );
            Ok(())
        } else {
            Err(ErrorCode::MascotaNoEncontrada.into())
        }
    }

}

#[error_code]
pub enum ErrorCode {
    #[msg("Mascota no encontrada en la veterinaria.")]
    MascotaNoEncontrada,
}

#[derive(Accounts)]
pub struct CrearVeterinaria<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + Veterinaria::INIT_SPACE,
        seeds = [b"veterinaria", owner.key().as_ref()],
        bump
        )] // Espacio estimado, ajusta mejor si quieres
    pub veterinaria: Account<'info, Veterinaria>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Veterinaria {
    #[max_len(100)]
    pub nombre: String,

    pub owner: Pubkey,

    #[max_len(10)]
    pub mascotas: Vec<Mascota>,
}

// Definimos una estructura simple de Mascota para almacenar algo básico, tú la puedes ampliar
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Mascota {
    #[max_len(60)]
    pub especie: String,

    #[max_len(60)]
    pub nombre: String,

    #[max_len(60)]
    pub owner: String,

    pub edad: u8,

    pub vivo: bool,
}

#[derive(Accounts)]
pub struct AgregarMascota<'info> {
    #[account(mut)]
    pub veterinaria: Account<'info, Veterinaria>,

    pub owner: Signer<'info>, // Opcionalmente, puedes usar owner para validar permisos
}

#[derive(Accounts)]
pub struct EliminarRegistroMascota<'info> {
    #[account(mut)]
    pub veterinaria: Account<'info, Veterinaria>,

    pub owner: Signer<'info>, // para validar permisos del dueño
}

#[derive(Accounts)]
pub struct VerMascotas<'info> {
    pub veterinaria: Account<'info, Veterinaria>,
}

#[derive(Accounts)]
pub struct CambiarEstado<'info> {
    #[account(mut)]
    pub veterinaria: Account<'info, Veterinaria>,

    pub owner: Signer<'info>, // para validar que sea el dueño quien cambia el estado
}
