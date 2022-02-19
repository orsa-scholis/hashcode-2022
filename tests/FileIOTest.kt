import org.testng.annotations.AfterClass
import org.testng.annotations.BeforeClass
import org.testng.annotations.Test
import java.io.File
import kotlin.test.assertContentEquals
import kotlin.test.assertEquals
import kotlin.test.assertNotNull

internal class FileIOTest {
    companion object {
        lateinit var testFiles: Array<File>
        lateinit var testFileNames: Array<String>

        @BeforeClass @JvmStatic fun setup() {
            val fileA = File("tests/data/a_example.txt")
            fileA.createNewFile()
            val fileB = File("tests/data/b_first.txt")
            fileB.createNewFile()
            val fileC = File("tests/data/c_second.txt")
            fileC.createNewFile()

            testFiles = arrayOf(fileA, fileB, fileC)
            testFileNames = testFiles.map { f: File -> f.name }.toTypedArray()
        }

        @AfterClass @JvmStatic fun teardown() {
            testFiles.onEach { file: File -> file.delete() }
        }
    }

    @Test
    fun list() {
        val result = FileIO.list("tests/data")

        assertNotNull(result)
        assertEquals(testFileNames.size, result.size)
        assertContentEquals(testFileNames, result)
    }

    @Test
    fun read() {
        val firstFile = testFiles.first()
        val text = "Hello World\nTest Test!!"
        val expectedTextList = text.split("\n")

        firstFile.writeText(text)

        val content = FileIO.read("tests/data/" + firstFile.name)
        assertEquals(expectedTextList, content);
    }

    @Test
    fun write() {
        val firstFile = testFiles.first()

        val textList = listOf("Hello World", "Test Test!", "Are we writing")
        val expectedText = textList.joinToString("\n")

        FileIO.write("tests/data/" + firstFile.name, textList)

        val content = firstFile.readText()
        assertEquals(expectedText, content)
    }
}