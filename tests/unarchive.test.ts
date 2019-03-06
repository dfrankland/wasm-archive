import { readFile } from 'fs';
import { promisify } from 'util';
import { resolve as resolvePath } from 'path';
import { unarchive } from '../pkg/wasm_archive';

const readFileAsync = promisify(readFile);

describe('unarchive', () => {
  it('unarchives zip files', async () => {
    const zipFile = await readFileAsync(
      resolvePath(__dirname, './fixtures.zip'),
    );
    const files = unarchive(zipFile);
    expect(files).toEqual([
      { path: 'fixtures', buffer: [] },
      { path: 'fixtures/b.txt', buffer: [...Buffer.from('b\n')] },
      { path: 'fixtures/a.txt', buffer: [...Buffer.from('a\n')] },
    ]);
  });

  it('unarchives tar files', async () => {
    const tarFile = await readFileAsync(
      resolvePath(__dirname, './fixtures.tar'),
    );
    const files = unarchive(tarFile);
    expect(files).toEqual([
      { path: './fixtures/', buffer: [] },
      { path: './fixtures/b.txt', buffer: [...Buffer.from('b\n')] },
      { path: './fixtures/a.txt', buffer: [...Buffer.from('a\n')] },
    ]);
  });

  it('throws for unknown files', () => {
    expect(() => unarchive(Buffer.from([1, 2, 3]))).toThrowError(
      'Unknown file type for buffer:\n[1, 2, 3]',
    );
  });
});
