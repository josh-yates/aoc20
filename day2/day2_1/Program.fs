open System.IO

module aocFns =
    // https://stackoverflow.com/a/40039995
    let countChar (getStr : string)(chkdChar : char) = 
        let rec loop i count =
            if i < getStr.Length then 
                if getStr.[i] = chkdChar then loop (i+1) (count+1)
                else loop (i+1) count
            else count
        loop 0 0
    
    let parseLine (line:string) = 
        let splitLine = line.Split ' '
        let range = splitLine.[0].Split '-'
        let min = int range.[0]
        let max = int range.[1]
        let letter = splitLine.[1].[0]
        let password = splitLine.[2]

        let charCount = countChar password letter
        charCount >= min && charCount <= max

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

