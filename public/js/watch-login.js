$(function() {
    var ctx = document.getElementById("myChart");
    var chardData = $(ctx).data('chart-data');
    var labels = $.map(chardData, function(x) { return x.date; });
    var failedIpCounts = $.map(chardData, function(x) {
        var n = 0;
        $.map(x.failed, function() { ++n });
        return n;
    });
    var failedCounts = $.map(chardData, function(x) {
        var n = 0;
        $.map(x.failed, function(v) { n += v });
        return n;
    });
    var myChart = new Chart(ctx, {
        //type: 'bar',
        type: 'line',
        data: {
            labels: labels,
            datasets: [
                { label: 'ログイン失敗IP数',
                  data: failedIpCounts,
                  borderWidth: 1
                },
                { label: 'ログイン失敗数',
                  data: failedCounts,
                  borderWidth: 1
                }
            ]
        },
        options: {
            scales: {
                yAxes: [{
                    ticks: {
                        beginAtZero:true
                    }
                }]
            }
        }
    });
});
