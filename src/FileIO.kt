import java.io.*
import java.util.*

object FileIO {
    fun list(path: String = "data"): Array<String>? {
        val f = File(path)
        val ff = FilenameFilter { _: File?, name: String -> name.endsWith(".txt") }
        val list = f.list(ff)
        if (list != null) {
            Arrays.sort(list)
        }
        return list
    }

    @Throws(FileNotFoundException::class)
    fun read(path: String?): List<String> {
        val sc = Scanner(File(path))
        val lines: MutableList<String> = ArrayList()
        while (sc.hasNextLine()) {
            val line = sc.nextLine()
            lines.add(line)
        }
        sc.close()
        return lines
    }

    @Throws(IOException::class)
    fun write(path: String?, content: List<String?>) {
        val writer = BufferedWriter(FileWriter(path))
        writer.write(content.joinToString("\n"))
        writer.close()
    }
}