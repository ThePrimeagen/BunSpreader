export default function createBuffer(size: number): Uint8Array {
    const buffer8 = new Uint8Array(size);

    crypto.getRandomValues(buffer8);

    return buffer8;
}
