defmodule Day1 do
  def run_pt_1 do
    {left, right} =
      File.stream!("./lib/input.txt")
      |> Enum.reduce({[], []}, fn line, {l, r} ->
        {num_1, num_2} = parse_line(line)
        {[num_1 | l], [num_2 | r]}
      end)

    left = Enum.sort(left)
    right = Enum.sort(right)

    Enum.zip(left, right)
    |> Enum.reduce(0, fn {num_1, num_2}, acc ->
      acc + distance(num_1, num_2)
    end)
    |> IO.inspect(label: :result)
  end

  # Bit of a cheat here since I'm relying on the fact that all the numbers are the exact
  # same number of digits and the spaces are always the same. This is a brittle approach for fun
  # since pattern matching really efficient some fun code golf or whatever
  defp parse_line(
         <<num_1_raw::binary-size(5), _::binary-size(3), num_2_raw::binary-size(5), "\n">>
       ) do
    {String.to_integer(num_1_raw), String.to_integer(num_2_raw)}
  end

  defp parse_line(bad_line) do
    IO.puts("bad line #{bad_line}")
  end

  defp distance(num_1, num_2) do
    abs(num_1 - num_2)
  end

  def run_pt_2 do
    {left, right} =
      File.stream!("./lib/input.txt")
      |> Enum.reduce({[], []}, fn line, {left, right} ->
        {left_num, right_num} = parse_line(line)

        {[left_num | left], [right_num | right]}
      end)

    frequencies = Enum.frequencies(right)

    result =
      Enum.reduce(left, 0, fn num, acc ->
        number_score = num * Map.get(frequencies, num, 0)
        acc + number_score
      end)

    IO.inspect(result, label: :result)
  end
end
