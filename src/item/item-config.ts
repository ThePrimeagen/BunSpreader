export type ItemConfig = {
    girth: number;
    depth: number;
    value: number;
    bit: number;
};

export default function hasBackground({ value, bit }: ItemConfig): boolean {
    const offsetBit = 0x1 << Math.max(0, bit - 1);
    return Boolean(value & offsetBit);
}
