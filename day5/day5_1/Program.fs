open System.IO

module aocFns =
    let readLines (filePath:string) = seq<string> {
        use sr = new StreamReader (filePath)
        while not sr.EndOfStream do
            yield sr.ReadLine ()
    }

    let rec bisect (code:string, highChar:char, lowChar:char, high:int, low:int) =

        let nextChar = code.[0]

        if (high - low).Equals 1 then
            if nextChar.Equals highChar then
                high
            else
                low
        else if (high - low).Equals 2 then
            high - 1
        else

        let midPoint = int ((float low) + ((float (high - low)) / 2.0))

        if nextChar.Equals highChar then
            bisect (code.Substring(1), highChar, lowChar, high, midPoint)
        else
            bisect (code.Substring(1), highChar, lowChar, midPoint, low)
    
    let calcCode (line:string) =
            let row = bisect (line.Substring(0, 7), 'B', 'F', 127, 0)
            let col = bisect (line.Substring(7), 'R', 'L', 7, 0)

            ((row * 8) + col)

[<EntryPoint>]
let main argv =
    let highestCode = aocFns.readLines "/home/josh/Downloads/day5"
                    |> Seq.map(aocFns.calcCode)
                    |> Seq.sortDescending
                    |> Seq.item(0)  

    printfn "Answer: %d" highestCode

    0 // return an integer exit code
