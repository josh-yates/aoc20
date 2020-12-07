open System.IO

module aocFns =
    let readLines (filePath:string) = seq<string> {
        use sr = new StreamReader (filePath)
        while not sr.EndOfStream do
            yield sr.ReadLine ()
    }

    let calcCode (row:int, col:int) =
        (row * 8 + col)

    let getAllCodes(rows:int, cols:int) = seq<int> {
        for r = 0 to rows - 1 do
            for c = 0 to cols - 1 do
                yield calcCode (r, c)
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

        let midPoint = ((double low) + ((double (high - low)) / 2.0))

        if nextChar.Equals highChar then
            bisect (code.Substring(1), highChar, lowChar, high, int (System.Math.Ceiling midPoint))
        else
            bisect (code.Substring(1), highChar, lowChar, int (System.Math.Floor midPoint), low)

    
    let calcCodeForLine (line:string) =
        let row = bisect (line.Substring(0, 7), 'B', 'F', 127, 0)
        let col = bisect (line.Substring(7), 'R', 'L', 7, 0)

        calcCode (row, col)
    
    let getAssignedCodes () = seq<int> {
        for line in readLines "/home/josh/Downloads/day5" do
            yield calcCodeForLine(line)
    }

[<EntryPoint>]
let main argv =
    let allCodes = aocFns.getAllCodes (128, 8)
    let assignedCodes = aocFns.getAssignedCodes()

    let missingCodes = allCodes
                       |> Seq.except assignedCodes
                       |> Seq.where (fun x -> x < ((assignedCodes) |> Seq.max))
                       |> Seq.where (fun x -> x > ((assignedCodes) |> Seq.min))

    for code in missingCodes do
        let hasPrior = assignedCodes
                       |> Seq.contains (code - 1)
        let hasNext = assignedCodes
                      |> Seq.contains (code + 1)
        
        if hasNext && hasPrior then
            printfn "%d" code

    0 // return an integer exit code
