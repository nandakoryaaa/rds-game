/**
 * ZLIB DEFLATE decompressor
 * derived from Python code at https://pyokagan.name/blog/2019-10-18-zlibinflate/
 */

struct LZTables {
    length_extra_bits: [u8; 29],
    length_base: [u16; 29],
    dist_extra_bits: [u8; 30],
    dist_base: [u16; 30],
    code_length_codes_order: [u8; 19]
}

const LZT: LZTables = LZTables {
    length_extra_bits: [
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3,
        4, 4, 4, 4, 5, 5, 5, 5, 0
    ],
    length_base: [
        3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31,
        35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258
    ],
    dist_extra_bits: [
        0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8,
        8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13
    ],
    dist_base: [
        1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193,
        257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193,
        12289, 16385, 24577
    ],
    code_length_codes_order: [
        16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15
    ]
};

struct BitReader<'a> {
    mem: &'a [u8],
    pos: usize,
    b: u8,
    numbits: u8
}

impl<'a> BitReader<'a> {
    fn init(mem: &'a [u8]) -> Self {
        BitReader {
            mem: mem,
            pos: 0,
            b: 0,
            numbits: 0
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.numbits = 0; // discard unread bits
        let b = self.mem[self.pos];
        self.pos += 1;

        b
    }

    fn read_bit(&mut self) -> u8 {
        if self.numbits < 1 {
            self.b = self.read_byte();
            self.numbits = 8;
        }
        self.numbits -= 1;
        let bit = self.b & 1;
        self.b >>= 1;

        bit
    }

    fn read_bits(&mut self, n: u8) -> u64 {
        let mut o: u64 = 0;
        for i in 0..n {
            o |= (self.read_bit() as u64) << i;
        }

        o
    }

    fn read_bytes(&mut self, n: u8) -> u64 {
        // read bytes as an integer in little-endian
        let mut o: u64 = 0;
        for i in 0..n {
            o |= (self.read_byte() as u64) << (8 * i);
        }

        o
    }
}

pub fn decompress(input: &[u8]) -> Vec<u8> {
    let mut r = BitReader::init(input);
    let cmf = r.read_byte();
    let cm = cmf & 15; // Compression method
    if cm != 8 { // only CM = 8 is supported
        panic!("invalid CM");
    }
    let cinfo = (cmf >> 4) & 15; // Compression info
    if cinfo > 7 {
        panic!("invalid CINFO");
    }
    let flg = r.read_byte() as u32;
    if (cmf as u32 * 256 + flg) % 31 != 0 {
        panic!("CMF + FLG checksum failed");
    }
    let fdict = (flg >> 5) & 1; // preset dictionary?
    if fdict != 0 {
        panic!("preset dictionary not supported");
    }
    let out = inflate(&mut r); // decompress DEFLATE data
    let _adler32 = r.read_bytes(4); // Adler-32 checksum (for this exercise, we ignore it)

    out
}

fn inflate(r: &mut BitReader) -> Vec<u8> {
    let mut bfinal = 0;
    let mut out = Vec::<u8>::new();
    while bfinal == 0 {
        bfinal = r.read_bit();
        let btype = r.read_bits(2);
        if btype == 0 {
            inflate_block_no_compression(r, &mut out);
        } else if btype == 1 {
            inflate_block_fixed(r, &mut out);
        } else if btype == 2 {
            inflate_block_dynamic(r, &mut out);
        } else {
            panic!("invalid BTYPE");
        }
    }

    out
}

fn inflate_block_no_compression(r: &mut BitReader, o: &mut Vec<u8>) {
    let len = r.read_bytes(2);
    let _nlen = r.read_bytes(2);
    for _ in 0..len {
        o.push(r.read_byte());
    }
}

#[derive (Copy, Clone)]
struct Node {
    symbol: u16,
    left: usize,
    right: usize
}

impl Node {
    fn init() -> Self {
        Self {
            symbol: 0,
            left: 0,
            right: 0
        }
    }
}

struct HuffmanTree {
    storage: Vec<Node>
}

impl HuffmanTree {
    fn init() -> Self {
        let mut tree = Self {
            storage: Vec::<Node>::new()
        };
        tree.alloc_node(Node::init());
        tree
    }

    fn alloc_node(&mut self, node: Node) -> usize {
        let index: usize = self.storage.len();
        self.storage.push(node);
        index
    }

    fn get_node(&self, index: usize) -> Node {
        self.storage[index]
    }

    fn insert(&mut self, codeword: u64, n: u16, symbol: u16) {
        // Insert an entry into the tree mapping `codeword` of len `n` to `symbol`
        let mut node_index = 0;
        for i in (0..n).rev() {
            let mut node = self.get_node(node_index);
            if codeword & (1 << i) != 0 {
                if node.right == 0 {
                    node.right = self.alloc_node(Node::init());
                    self.storage[node_index].right = node.right;
                }
                node_index = node.right;
            } else {
                if node.left == 0 {
                    node.left = self.alloc_node(Node::init());
                    self.storage[node_index].left = node.left;
                }
                node_index = node.left;
            }
        }
        self.storage[node_index].symbol = symbol;
    }
}

fn inflate_block_dynamic(r: &mut BitReader, o: &mut Vec<u8>) {
    let (literal_length_tree, distance_tree) = decode_trees(r);
    inflate_block_data(r, &literal_length_tree, &distance_tree, o);
}

fn inflate_block_fixed(r: &mut BitReader, o: &mut Vec<u8>) {
    let mut bl: Vec<u16> = [
        vec![8; 144],
        vec![9; 256 - 144],
        vec![7; 280 - 256],
        vec![8; 288 - 280],
    ].concat();

    let mut range: Vec<u16> = (0..286).collect();
    let literal_length_tree = bl_list_to_tree(&bl, &range);

    bl = vec![5; 30];
    range = (0..30).collect();
    let distance_tree = bl_list_to_tree(&bl, &range);

    inflate_block_data(r, &literal_length_tree, &distance_tree, o);
}

fn bl_list_to_tree(bl: &[u16], alphabet: &Vec<u16>) -> HuffmanTree {
    // Find maximum value in bl
    let max_bits = bl.iter().max().unwrap_or(&0);
    // Count occurrences of each bit length (1..max_bits)
    let mut bl_count = vec![0; (max_bits + 1) as usize];
    for x in bl {
        if *x != 0 {
            bl_count[*x as usize] += 1;
        }
    }

    let mut next_code: Vec<u64> = vec![0, 0];

    for bits in 2..max_bits + 1 {
        next_code.push((next_code[bits as usize - 1] + bl_count[bits as usize - 1]) << 1);
    }
    let mut t = HuffmanTree::init();

    for (c, bitlen) in alphabet.into_iter().zip(bl) {
        if *bitlen != 0 {
            t.insert(next_code[*bitlen as usize], *bitlen, *c);
            next_code[*bitlen as usize] += 1;
        }
    }

    t
}

fn decode_symbol(r: &mut BitReader, t: &HuffmanTree) -> u16 {
    // Decodes one symbol from bitstream `r` using HuffmanTree `t`
    let mut node_index = 0;
    loop {
        let node = t.get_node(node_index);
        if node.left == 0 && node.right == 0 {
            break;
        }
        let b = r.read_bit();
        if b != 0 {
            node_index = node.right;
        } else {
            node_index = node.left;
        }
    }
    t.get_node(node_index).symbol
}

fn inflate_block_data(
    r: &mut BitReader, literal_length_tree: &HuffmanTree,
    distance_tree: &HuffmanTree, out: &mut Vec<u8>
) {
    loop {
        let mut sym = decode_symbol(r, literal_length_tree);
        if sym <= 255 { // Literal byte
            out.push(sym as u8);
        } else if sym == 256 { // End of block
            return;
        } else { // <length, backward distance> pair
            sym -= 257;
            let length = r.read_bits(LZT.length_extra_bits[sym as usize])
                + LZT.length_base[sym as usize] as u64;
            let dist_sym = decode_symbol(r, distance_tree) as usize;
            let dist = r.read_bits(LZT.dist_extra_bits[dist_sym])
                + LZT.dist_base[dist_sym] as u64;
            for _ in 0..length {
                out.push(out[out.len() - dist as usize]);
            }
        }
    }
}

fn decode_trees(r: &mut BitReader) -> (HuffmanTree, HuffmanTree) {
    // The number of literal/length codes
    let hlit = r.read_bits(5) + 257;

    // The number of distance codes
    let hdist = r.read_bits(5) + 1;

    // The number of code length codes
    let hclen = r.read_bits(4) + 4;

    // Read code lengths for the code length alphabet
    let mut code_length_tree_bl: Vec<u16> = vec![0; 19];
    for i in 0..hclen {
        code_length_tree_bl[
            LZT.code_length_codes_order[i as usize] as usize
        ] = r.read_bits(3) as u16;
    }

    // Construct code length tree
    let mut range: Vec<u16> = (0..19).collect();
    let code_length_tree = bl_list_to_tree(&code_length_tree_bl, &range);

    // Read literal/length + distance code length list
    let mut bl = Vec::<u16>::new();
    while bl.len() < (hlit + hdist) as usize {
        let sym = decode_symbol(r, &code_length_tree);
        if sym <= 15 { // literal value
            bl.push(sym);
        } else if sym == 16 {
            // copy the previous code length 3..6 times.
            // the next 2 bits indicate repeat length ( 0 = 3, ..., 3 = 6 )
            let prev_code_length = bl[bl.len() - 1];
            let repeat_length = r.read_bits(2) + 3;
            for _ in 0..repeat_length {
                bl.push(prev_code_length);
            }
        } else if sym == 17 {
            // repeat code length 0 for 3..10 times. (3 bits of length)
            let repeat_length = r.read_bits(3) + 3;
            for _ in 0..repeat_length {
                bl.push(0);
            }
        } else if sym == 18 {
            // repeat code length 0 for 11..138 times. (7 bits of length)
            let repeat_length = r.read_bits(7) + 11;
            for _ in 0..repeat_length {
                bl.push(0);
            }
        } else {
            panic!("invalid symbol");
        }
    }

    // Construct trees
    range = (0..286).collect();
    let literal_length_tree = bl_list_to_tree(&bl[..hlit as usize], &range);
    range = (0..30).collect();
    let distance_tree = bl_list_to_tree(&bl[hlit as usize..], &range);

    (literal_length_tree, distance_tree)
}
