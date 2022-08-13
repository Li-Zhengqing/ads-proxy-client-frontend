import * as echarts from 'echarts';
import React from 'react';

class ForceChart extends React.Component {
  constructor(props) {
    super(props);
  }
  componentDidMount() {
    // Temporary Option
    let option = this.renderChart();

    this.chart = echarts.init(document.getElementById(this.props.chartId));
    this.chart.setOption(option);
  }
  renderChart() {
    // init option
    let option = {
      tooltip: {
        trigger: 'axis',
        position: function (pt) {
          return [pt[0], '10%'];
        }
      },
      toolbox: {
        feature: {
          dataZoom: {
            yAxisIndex: 'none'
          },
          // restore: {},
          saveAsImage: {}
        }
      },
      dataZoom: [
        {
          type: 'inside',
          start: 0,
          end: 100
        },
        {
          start: 0,
          end: 100
        }
      ],
      animation: false,
      xAxis: {
        type: 'category',
        data: this.props.data.x
      },
      yAxis: {
        type: 'value',
      },
      legend: {
        // orient: 'vertical',
        // right: 10,
        // top: 'center',
        data: this.props.data.y.map((item) => (item.name))
      },
      title: {
        left: 'center',
        text: this.props.title
      },
      /* series: [
        {
          data: this.props.data.y,
          type: 'line',
          color: this.props.color
        }
      ] */
      series: this.props.data.y.map((item) => ({
        data: item.data,
        type: 'line',
        color: item.color
      }))
    };

    // FIXME: Not so elegant, maybe can be solve by using a timer
    if (this.chart !== undefined) {
      this.chart.setOption(option);
    }

    return option;
  }
  render() {
    this.renderChart();
    return (
      // <div id={this.props.chartId} style={{width: 500, height: 300}}></div>
      <div id={this.props.chartId} style={{width: this.props.size.width, height: this.props.size.height}}></div>
    );
  }
}

export default ForceChart;
