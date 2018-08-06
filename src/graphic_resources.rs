use opengl_graphics::{ GlyphCache, TextureSettings };

pub struct GraphicResources<'a> {
    pub roboto_glyph_cache: GlyphCache<'a>
}

impl<'a> GraphicResources<'a> {
    pub fn load() -> GraphicResources<'a> {
 
        GraphicResources {
            roboto_glyph_cache: GlyphCache::new("resources/fonts/roboto-light.ttf", (), 
                TextureSettings::new()).expect("Failed to load the glyph cache roboto-light")
        }
    }
}