<?php

namespace Nano\Phpfd\Tests\Benchmarks;

class GetFDBenchmark
{
    private string $file = __DIR__ . '/file.txt';
    private $resource = null;

    public function __construct()
    {
        $this->resource = fopen($this->file, 'w');
    }

    /**
     * @Revs(1000)
     * @Iterations(5)
     */
    public function bench_phpfd_extension()
    {
        $fd = resource_fd($this->resource);
    }

    /**
     * @Revs(1000)
     * @Iterations(5)
     */
    public function bench_old_way()
    {
        $fd = self::getFD($this->file);
    }

    public static function getFD(string $fullPathToFile): int
    {
        $path = $fullPathToFile;

        if (is_resource($fullPathToFile)) {
            $path = stream_get_meta_data($fullPathToFile)['uri'];
        }

        if (!is_string($path)) {
            return -1;
        }

        $dir = '/proc/self/fd/';
        $dh = opendir($dir);

        if ($dh === false) {
            return -1;
        }

        while (($file = readdir($dh)) !== false) {
            $filename = $dir . $file;
            if (filetype($filename) === 'link' && realpath($filename) === $path) {
                closedir($dh);
                return (int)$file;
            }
        }

        closedir($dh);
        return -1;
    }

    public function __destruct()
    {
        fclose($this->resource);
        unlink($this->file);
    }
}
