import {useContext} from "react";
import Model from "../types/Model";
import ModelContext from "../contexts/ModelContext";
import {BenchmarkType} from "../types/Model";
import {benchmark_type_str, equation_str, is_read_op_str} from "../utils/ModelUtils";
import Formula from "../components/Formula";

type PlotViewProps = {
  benchmark_type: BenchmarkType,
  is_read_op: boolean,
}

const PlotView = ({benchmark_type, is_read_op}: PlotViewProps) => {
  // TODO NULL OPERATOR
  const model: Model = useContext(ModelContext)!.json;
  const ourModel = model.find(el => el.benchmark_type === benchmark_type && el.is_read_op === is_read_op)!;
  return (
    <div className="mx-auto max-w-lg">
      <h1 className="text-center text-4lg">{benchmark_type_str(benchmark_type)}: {is_read_op_str(is_read_op)} Operations</h1>
      {/* Formula */}
      <div>
        <Formula tex={equation_str(ourModel.model)} />
      </div>
      {/* Functinon plot */}
      {/* Plotting of each KDE */}
    </div>
  );
}

export default PlotView;
