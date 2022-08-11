import logo from './logo.svg';
import './App.css';
import * as echarts from 'echarts';
import { Button } from '@mui/material';
import AppBar from '@mui/material/AppBar';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import Stack from '@mui/material/Stack';
// import Icon from '@mui/material';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import StopIcon from '@mui/icons-material/Stop';
import FiberManualRecordIcon from '@mui/icons-material/FiberManualRecord';
import FiberSmartRecordIcon from '@mui/icons-material/FiberSmartRecord';
import React from 'react';

class ForceChart extends React.Component {
  constructor(props) {
    super(props);
  }
  componentDidMount() {
    // Temporary Option
    let option = {
      xAxis: {
        type: 'category',
        data: this.props.data.x
      },
      yAxis: {
        type: 'value',
      },
      series: [
        {
          data: this.props.data.y,
          type: 'line',
          color: this.props.color
        }
      ]
    };

    let chart = echarts.init(document.getElementById(this.props.chartId));
    chart.setOption(option);
  }
  render() {
    return (
      <div id={this.props.chartId} style={{width: 700, height: 300}}></div>
    );
  }
}

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
      <AppBar>
        <Toolbar>
          <Typography variant="h5" component="h5">Cutting Force Monitor</Typography>
        </Toolbar>
      </AppBar>
      <Button variant="contained" color="success" size="large">
        Start
        <PlayArrowIcon></PlayArrowIcon>  
      </Button>
      <Button variant="contained" color="error" size="large">
        Stop
        <StopIcon></StopIcon>
      </Button>
      <Button variant="contained" color="warning" size="large">
        Record
        <FiberManualRecordIcon></FiberManualRecordIcon>
      </Button>
      <Stack horizon="row" spacing={2}>
        <ForceChart chartId="Fc" data={{
          x: [1, 2, 3, 4, 5, 6, 7],
          y: [150, 230, 224, 218, 135, 147, 260]
        }} color="#ff0000"></ForceChart>
        <ForceChart chartId="Ff" data={{
          x: [1, 2, 3, 4, 5, 6, 7],
          y: [150, 230, 224, 218, 135, 147, 260]
        }} color="#0000ff"></ForceChart>
        <ForceChart chartId="Fp" data={{
          x: [1, 2, 3, 4, 5, 6, 7],
          y: [150, 230, 224, 218, 135, 147, 260]
        }} color="#00ff00"></ForceChart>
      </Stack>
    </div>
  );
}

export default App;
