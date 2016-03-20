$(function() {

    var dataset = $('[data-dataset]').data('dataset');

    var w = 600;
    var h = 100;
    var barPadding = 1;  // <-- パディング（棒の間の間隔）を追加！

    var yScale = d3.scale.linear()
        .domain([0, d3.max(dataset)])
        .range([0, h]);

    var yFillScale = d3.scale.linear()
        .domain([0, d3.max(dataset)])
        .range([0, 255]);



    // SVG 要素の生成
    var svg = d3.select("#graph")
        .append("svg")
        .attr("width", w)
        .attr("height", h);

    svg.selectAll("rect")
        .data(dataset)
        .enter()
        .append("rect")
        .attr("x", function(d, i) {
            return i * (w / dataset.length);
        })
        .attr("y", function(d) {
            return h - yScale(d);  // SVG の高さからデータの値を引く
        })
        .attr("width", w / dataset.length - barPadding)
        .attr("height", function(d) {
            return yScale(d);
        })
        .attr("fill", function(d) {
            return "rgb(0, 0, " + Math.floor(yFillScale(d)) + ")";
        })
    ;

    svg.selectAll("text")
        .data(dataset)
        .enter()
        .append("text")
        .text(function(d) {
            return d;
        })
        .attr("text-anchor", "middle")
        .attr("x", function(d, i) {
            return i * (w / dataset.length) + (w / dataset.length - barPadding) / 2;
        })
        .attr("y", function(d) {
            return h - yScale(d) + 14;  // 15 を 14 に
        })
        .attr("font-family", "sans-serif")
        .attr("font-size", "11px")
        .attr("fill", "white")
    ;
});