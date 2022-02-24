fun main() {
    val list = FileIO.list()

//    print("first: ${list?.first()}")

    algo("data/${list?.first()!!}")

//    list?.forEach {
//        algo(it)
//    }
}

fun algo(filePath: String) {
    val content = FileIO.read(filePath)

    var contributorCount: Int? = null;
    val contributors: MutableList<Contributor> = mutableListOf();
    var projectsCount: Int? = null;

    var currentPersonSkillCount: Int? = null
    var currentPersonName: String? = null
    val currentPersonSkills: MutableList<Skill> = emptyList<Skill>().toMutableList();
    content.forEachIndexed { index, line ->
        if (index == 0) {
            val parts = line.split(" ")
            contributorCount = parts.first().toInt();
            projectsCount = parts.first().toInt();
        } else {
            if (contributors.size < contributorCount!!) {

                val parts = line.split(" ")

                if (currentPersonName == null) {
                    currentPersonName = parts[0]
                    currentPersonSkillCount = parts[1].toInt()
                }

                if (currentPersonSkills.size < currentPersonSkillCount!!) {
                    currentPersonSkills.add(Skill(parts[0], parts[1].toInt()))
                }

                if (currentPersonSkills.size == currentPersonSkillCount!!) {
                    contributors.add(Contributor(currentPersonName!!, currentPersonSkills))
                    currentPersonName = null
                }

            } else {
                // TOdo projects
            }
        }
    }

    print("Contributors: $contributors");
}






