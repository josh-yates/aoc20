open System.IO

module aocFns =
    let parseLine (line:string) = 
        let splitLine = line.Split ' '
        let range = splitLine.[0].Split '-'
        let pos1 = int range.[0] - 1
        let pos2 = int range.[1] - 1
        let letter = splitLine.[1].[0]
        let password = splitLine.[2]

        ((password.[pos1].Equals letter) && not (password.[pos2].Equals letter)) ||
        ((password.[pos2].Equals letter) && not (password.[pos1].Equals letter))

    let parseLines (filePath:string) = seq {
        use sr = new StreamReader (filePath)
        while not sr.EndOfStream do
            let line = sr.ReadLine ()
            yield parseLine line
    }


[<EntryPoint>]
let main argv =

    let parsedLines =
        aocFns.parseLines "/home/josh/Downloads/day2"
        |> Seq.filter(id)
    
    printfn "Answer: %d" (Seq.length parsedLines)

    0 // return an integer exit code

