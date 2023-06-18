use super::GameManager;

pub fn get_flags(game: &GameManager) -> String {
    format!(
        r#"javaw -Xms{}m -Xmx{}m -Djava.library.path=natives -cp "minecraft.jar;jinput.jar;lwjgl.jar;lwjgl_util.jar;" net.minecraft.client.Minecraft "{}" "{}" "#,
        game.min_use_memory.as_u64() / bytesize::MB,
        game.max_use_memory.as_u64() / bytesize::MB,
        game.username,
        ""
    )
}
