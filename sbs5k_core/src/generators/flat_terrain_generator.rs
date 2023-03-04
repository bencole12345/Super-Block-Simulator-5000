use crate::block::Block;
use crate::chunk::{
    empty_blocks, Chunk, ChunkIndex, ChunkSource, CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH,
};

/// A basic `ChunkSource` that just emits flat chunks containing a layer of grass, three layers of
/// dirt, and 61 layers of stone
#[derive(Default)]
pub struct FlatTerrainGenerator;

impl ChunkSource for FlatTerrainGenerator {
    fn get_chunk_at(&mut self, _coordinate: ChunkIndex) -> Box<Chunk> {
        let mut blocks = empty_blocks();
        #[allow(clippy::needless_range_loop)]
        for x in 0..CHUNK_WIDTH {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_DEPTH {
                    let block = if y > 64 {
                        Block::Empty
                    } else if y == 64 {
                        Block::Grass
                    } else if y > 60 {
                        Block::Dirt
                    } else {
                        Block::Stone
                    };
                    blocks[x][y][z] = block;
                }
            }
        }
        Box::new(Chunk::new(blocks))
    }
}
